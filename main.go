package main

import (
	"lynixapi/api/events"
	"lynixapi/api/fursuit"
	"lynixapi/database"
	"lynixapi/database/models"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"

	"log"
)

func initDatabase() {
	database.Connect()
	database.Database.AutoMigrate(&models.Event{})
	database.Database.AutoMigrate(&models.WolfHR{})
	database.Database.AutoMigrate(&models.Fursuit{})
}

func loadEnv() {
	err := godotenv.Load(".env")
	if err != nil {
		log.Fatal("Error loading .env file")
	}
}

func main() {
	r := gin.Default()

	// Load environment variables
	loadEnv()

	// Connect to the database
	initDatabase()

	r.GET("/", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"notice":  "WARNING: This API is currently in development and is not ready for production use.",
			"version": "LynixAPI v3.0.0_ALPHA (Matcha)",
		})
	})

	// Routes
	r.GET("/fursuit/get_data", fursuit.GetData)
	r.GET("/fursuit/get_data/:neos_username", fursuit.GetUserData)
	r.GET("/events", events.GetEvents)
	r.GET("/events/:id", events.FindEvent)
	r.POST("/events/add", events.AddEvent)
	r.POST("/events/update", events.UpdateEvent)
	r.POST("/events/delete", events.DeleteEvent)

	// Default 404 Route
	r.NoRoute(func(c *gin.Context) {
		c.JSON(404, gin.H{
			"error": "404 Not Found",
		})
	})

	// Run on port 8557 which is default for LynixAPI
	r.Run(":8557")
}
