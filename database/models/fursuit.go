package models

import (
	uuid "github.com/satori/go.uuid"
	"gorm.io/gorm"
)

type Fursuit struct {
	gorm.Model
	ID             uuid.UUID `gorm:"type:uuid;primary_key;"`
	NeosUsername   string    `json:"neos_username"`
	VRChatUsername string    `json:"vrchat_username"`

	// Fursuit Information
	// This feature was dropped as the FurSyncBoard are not connected to the internet for pollution reasons.
	// This would cause too much interfernce at Conventions and Hotels.

	// Fursuit RGB Control
	// Deprecated Feature: RGB Control will use a Mic Array to sync up with music and seperate into 5 channels
	// The idea was dropped due to Access Points in hotels not handling the stress-test correctly.
	// Other devices (FurSyncBoards) will be able to communicate with other fursuiters to sync up with music.

	// Fursuit Tail Control Cordinate Vector X, Y, Z with no Frame
	TailCordX int `json:"tail_cord_x"`
	TailCordY int `json:"tail_cord_y"`
	TailCordZ int `json:"tail_cord_z"`

	// Fursuit Cooling Information
	CoolingStatus string `json:"cooling_status"`
	CoolingTemp   int    `json:"cooling_temp"`
	OutsideTemp   int    `json:"outside_temp"`
}
