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
	r.GET("/events", events.GetEvents)

	// Default 404 Route
	r.NoRoute(func(c *gin.Context) {
		c.JSON(404, gin.H{
			"error": "404 Not Found",
		})
	})

	// Run on port 8557 which is default for LynixAPI
	r.Run(":8557")
}
