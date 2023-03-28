package api

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"lynixapi/database"
	"lynixapi/database/models"
)

func GetCertifications(ctx *gin.Context) {
	var certs []models.Certification
	database.Database.Find(&certs)

	ctx.JSON(200, certs)
}

func FindCertification(ctx *gin.Context) {
	var cert models.Certification

	if err := database.Database.Where("id = ?", ctx.Param("id")).First(&cert).Error; err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": "Certification not found!"})
		return
	}

	ctx.JSON(http.StatusOK, cert)
}
