package models

import "gorm.io/gorm"

type WolfHR struct {
	gorm.Model
	ID             string  `json:"string_id"`
	NeosUsername   string  `json:"neos_username"`
	VRChatUsername string  `json:"vrchat_username"`
	HeartRate      int     `json:"heartrate"`
	BatteryHealth  float64 `json:"battery_health"`
	LastUpdated    string  `json:"last_updated"`
}
