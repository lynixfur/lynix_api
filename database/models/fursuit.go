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
	FursuitSWVersion string `json:"fursuit_sw_version"`
	FursuitLIP       string `json:"fursuit_lip"`

	// Fursuit RGB Control
	PrevFrameRed   int `json:"prev_frame_red"`
	PrevFrameGreen int `json:"prev_frame_green"`
	PrevFrameBlue  int `json:"prev_frame_blue"`

	CurrentFrameRed   int `json:"current_frame_red"`
	CurrentFrameGreen int `json:"current_frame_green"`
	CurrentFrameBlue  int `json:"current_frame_blue"`

	NextFrameRed   int `json:"next_frame_red"`
	NextFrameGreen int `json:"next_frame_green"`
	NextFrameBlue  int `json:"next_frame_blue"`

	// Fursuit Tail Control Cordinate Vector X, Y, Z with no Frame
	TailCordX int `json:"tail_cord_x"`
	TailCordY int `json:"tail_cord_y"`
	TailCordZ int `json:"tail_cord_z"`

	// Fursuit Cooling Information
	CoolingStatus string `json:"cooling_status"`
	CoolingTemp   int    `json:"cooling_temp"`
	OutsideTemp   int    `json:"outside_temp"`
}
