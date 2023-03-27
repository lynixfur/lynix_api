package models

import (
	"time"

	uuid "github.com/satori/go.uuid"
	"gorm.io/gorm"
)

type Event struct {
	gorm.Model
	ID               uuid.UUID `gorm:"type:uuid;primary_key;"`
	EventName        string    `json:"event_name"`
	EventDescription string    `json:"event_description"`
	EventType        string    `json:"event_type"`
	EventImage       string    `json:"event_image"`
	Location         string    `json:"location"`
	Status           string    `json:"status"`
	StartDate        time.Time `json:"start_date"`
	EndDate          time.Time `json:"end_date"`
}
