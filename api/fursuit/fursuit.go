package fursuit

import (
	"lynixapi/database"
	"lynixapi/database/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

// This module is used for the Fursuit System that controls RGB lighting, tail animations and cooling on a fursuit.

func GetData(ctx *gin.Context) {
	ctx.JSON(403, gin.H{
		"message": "You don't have access to this endpoint.",
		"notice":  "This resource is only available lynix at the current moment, this feature may come to other fursuiters in the future! Stay tuned for announcements for when the FurSystem releases!",
	})
}

func GetUserData(ctx *gin.Context) {
	// Get Single Row from Database.
	var fursuit_data models.Fursuit

	if err := database.Database.Where("neos_username = ?", ctx.Param("neos_username")).First(&fursuit_data).Error; err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": "Fursuit data is not available, either the username or data is not initiated."})
		return
	}

	ctx.JSON(http.StatusOK, fursuit_data)
}

func updateData() {
	// Update data in the database.
}

func setTailControl() {
	// Set the tail control to the specified value.
}

func setRGBControl() {
	// Set the RGB control to the specified value.
}

func setCoolingControl() {
	// Set the cooling control to the specified value.
}
