package models

import "gorm.io/gorm"

type Event struct {
	gorm.Model
	EventName        string `json:"event_name"`
	EventDescription string `json:"event_description"`
	Location         string `json:"location"`
	Status           string `json:"status"`
	StartDate        string `json:"start_date"`
	EndDate          string `json:"end_date"`
}
