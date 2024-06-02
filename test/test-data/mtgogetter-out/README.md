# Format examples of MTGO Getter output

## Scryfall API

MTGO Getter outputs scryfall bulk data as a JSON-file containing an array of `ScryfallCard` (defined in MTGO Getter) containing the data that is deemed relevant or potentially relevant for MTGO Collection Manager as of now.

`scryfall-card.json` is an example of a serialized `ScryfallCard`.

## State log

MTGO Getter maintains a [TOML file](./fetch_log.toml) where it stores information about when various data was last updated, and what is the next set to be released on MTGO.

MTGO Parser uses the next released set information to find out when this set is actually starting to be available on MTGO, the release date is not taken as an absolute truth, as they have often been subject to delays in the past. The MTGO Parser removes the next released set from the state log once it can confirm that the new set is on MTGO, which it confirms by looking for the set ID in the card definitions. Removing the set from the state log tells the MTGO Getter (next time it runs) that it should update that field with new set release date information.
