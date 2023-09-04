package mtgogetter

import (
	"encoding/json"
	"log"
	"os"
)

type ScryfallCard struct{
    Mtgo_id int32 `json:"mtgo_id"`
	Mtgo_foil_id int32 `json:"mtgo_foil_id"`
	Name string `json:"name"`
	Released_at string `json:"released_at"`
	Rarity string `json:"rarity"`
	Prices struct { // All nullable (deserialized as empty string if null)
		Usd string `json:"usd"`
		Usd_foil string `json:"usd_foil"`
		Eur string `json:"eur"`
		Eur_foil string `json:"eur_foil"`
		Tix string `json:"tix"`
	} `json:"prices"`
}

func ReadBulkData(fname string	) []ScryfallCard {
	// Read file to bytes
	file_data, err := os.ReadFile(fname)
	if err != nil {
		log.Println("Error reading file:", err)
	}
	// Unmarshal JSON
	var bulk_data []ScryfallCard
	if err := json.Unmarshal(file_data, &bulk_data); err != nil {
		log.Println("Error unmarshalling JSON:", err)
	}

	return bulk_data
}