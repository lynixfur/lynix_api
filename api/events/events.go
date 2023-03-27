package events

import (
	"github.com/gin-gonic/gin"

	"lynixapi/database"
	"lynixapi/database/models"
)

func GetEvents(ctx *gin.Context) {
	// Get events from the database.
	var events []models.Event
	database.Database.Find(&events)

	ctx.JSON(200, events)
}

func UpdateEvent(ctx *gin.Context) {
	// Update event in the database.

	ctx.JSON(200, gin.H{
		"message": "This endpoint is currently under development.",
	})
}

func DeleteEvent(ctx *gin.Context) {
	// Delete event from the database that is no longer needed.

	ctx.JSON(200, gin.H{
		"message": "This endpoint is currently under development.",
	})
}

func AddEvent(ctx *gin.Context) {
	// Add new event to the database that was previously not there.

	ctx.JSON(200, gin.H{
		"message": "This endpoint is currently under development.",
	})
}
