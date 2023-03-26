package events

import (
	"github.com/gin-gonic/gin"
)

func GetEvents(ctx *gin.Context) {
	// Get events from the database.

	// TODO: Add database query to get events.
	ctx.JSON(404, gin.H{
		"message": "No events found.",
	})
}

func updateEvent() {
	// Update event in the database.
}

func deleteEvent() {
	// Delete event from the database that is no longer needed.
}

func addEvent() {
	// Add new event to the database that was previously not there.
}
