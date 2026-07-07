# Contains translations for general plugin strings
# Keys are referenced in plugin.kdl

name = My Plugin

description = "Example plugin"

# Strings used by the plugin binary
# Not referenced in plugin.kdl, but by plugin code

incorrect-message = "Incorrect!"

num-apples-message =
    { $numApples ->
        [one] You have one apple.
       *[other] You have { $numApples } apples.
    }
