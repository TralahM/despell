package main

import (
	"fmt"
	"github.com/bensadeh/despell/config"
	"github.com/bensadeh/despell/stock"

	"github.com/bensadeh/despell/arguments"
	"github.com/bensadeh/despell/core"
	"github.com/bensadeh/despell/overrider"
)

const (
	MissingCommandKey = "default"
)

func main() {
	settings := arguments.GetInputConfig()
	overrides := overrider.GetOverrides()
	stockMappings := stock.GetStockMappings()
	defaultIcon := stock.GetDefaultIcon()

	icon := getIcon(settings.Command, overrides, stockMappings, defaultIcon)
	output := format(settings, icon)

	fmt.Println(output)
}

func format(settings *config.Settings, icon core.Icon) string {
	if settings.UseEmoji {
		return icon.Emoji
	}

	if settings.UseColor {
		return "#[fg=" + icon.Color + "]" + icon.Icon
	}

	return icon.Icon
}

func getIcon(key string, overrides, defaults map[string]core.Icon, defaultIcon core.Icon) core.Icon {
	if overridesIcon, ok := overrides[key]; ok {
		return overridesIcon
	}

	if defaultsIcon, ok := defaults[key]; ok {
		return defaultsIcon
	}

	if overridesIcon, ok := overrides[MissingCommandKey]; ok {
		return overridesIcon
	}

	return defaultIcon
}
