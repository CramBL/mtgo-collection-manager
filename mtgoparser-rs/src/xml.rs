use quick_xml::events::attributes::{AttrError, Attributes};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::path::Path;

/// The relevant card information that can be extracted from an MTGO .dek XML file.
#[derive(Debug, Clone, PartialEq)]
pub struct XmlCard {
    pub cat_id: String,
    pub quantity: String,
    pub name: String,
}

impl XmlCard {
    const CAT_ID_IDX: usize = 0;
    const QUANTITY_IDX: usize = 1;
    const NAME_IDX: usize = 3;

    pub fn new(cat_id: String, quantity: String, name: String) -> Self {
        Self {
            cat_id,
            quantity,
            name,
        }
    }

    // The XML card information is contained in the attributes of the <Cards> tag.
    // e.g.
    // <Cards CatID="235" Quantity="1" Sideboard="false" Name="Swamp" Annotation="0" />
    pub fn from_xml_cards_attrs(card_attrs: Attributes) -> Result<Self, AttrError> {
        let mut cat_id = String::new();
        let mut quantity = String::new();
        let mut name = String::new();

        for (index, att) in card_attrs.enumerate() {
            let att = att?;
            match index {
                Self::CAT_ID_IDX => cat_id.push_str(&String::from_utf8_lossy(&att.value)),
                Self::QUANTITY_IDX => quantity.push_str(&String::from_utf8_lossy(&att.value)),
                Self::NAME_IDX => name.push_str(&String::from_utf8_lossy(&att.value)),
                _ => (),
            }
        }
        debug_assert!(!cat_id.is_empty() && !quantity.is_empty() && !name.is_empty());
        Ok(Self::new(cat_id, quantity, name))
    }
}

pub fn parse_dek_xml(path: &Path) -> Result<Vec<XmlCard>, quick_xml::Error> {
    let mut reader = Reader::from_file(path).unwrap();
    let mut deck: Vec<XmlCard> = Vec::with_capacity(1024);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(e) => match e {
                Event::Empty(e_tag) => {
                    let name = e_tag.name();
                    debug_assert_eq!(name, QName(b"Cards"), "Expected any/all empty element tags in MTGO .dek XML to have name=Cards but got name={name:?}");

                    let card = XmlCard::from_xml_cards_attrs(e_tag.attributes())?;
                    deck.push(card);
                }
                // Here for visibility
                Event::Text(_)
                | Event::Comment(_)
                | Event::PI(_)
                | Event::DocType(_)
                | Event::Decl(_)
                | Event::End(_)
                | Event::Start(_)
                | Event::CData(_) => (),
                Event::Eof => break,
            },
            Err(err) => {
                eprintln!("{err}");
                return Err(err);
            }
        }
    }

    Ok(deck)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    #[test]
    fn test_parse_dek_xml_5_cards() -> TestResult {
        let path = Path::new(r"../test/test-data/mtgo/Full Trade List-small-5cards.dek");
        let deck = parse_dek_xml(path)?;
        assert_eq!(deck.len(), 5);
        assert_eq!(
            deck[0],
            XmlCard {
                cat_id: "1".to_string(),
                quantity: "453".to_string(),
                name: "Event Ticket".to_string()
            }
        );
        assert_eq!(
            deck[4],
            XmlCard {
                cat_id: "110465".to_string(),
                quantity: "1".to_string(),
                name: "Tranquil Cove".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_dek_xml_3000_cards() -> TestResult {
        let path = Path::new(r"../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");
        let deck = parse_dek_xml(path)?;
        assert_eq!(deck.len(), 3000);
        assert_eq!(
            deck[0],
            XmlCard {
                cat_id: "1".to_string(),
                quantity: "391".to_string(),
                name: "Event Ticket".to_string()
            }
        );
        assert_eq!(
            deck[2999],
            XmlCard {
                cat_id: "106509".to_string(),
                quantity: "1".to_string(),
                name: "Sawblade Scamp".to_string()
            }
        );
        Ok(())
    }
}
