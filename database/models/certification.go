package models

import (
	"time"

	uuid "github.com/satori/go.uuid"
	"gorm.io/gorm"
)

type Certification struct {
	gorm.Model
	ID              uuid.UUID `gorm:"type:uuid;primary_key;"`
	CertName        string    `json:"cert_name"`
	CertDescription string    `json:"cert_description"`
	CertImage       string    `json:"cert_image"`
	CertValidation  string    `json:"cert_validation"`
	BadgeImage      string    `json:"badge_image"`
	IssuedBy        string    `json:"issued_by"`
	EarntDate       time.Time `json:"earnt_date"`
}
