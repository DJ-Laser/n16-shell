Plugins are placed in <config dir>/plugins

A plugin is a folder, contains plugin.kdl, plugin binary, and resources

Example:

```
plugins/example_plugin:
  plugin.kdl
  plugin
  translations:
    en-us:
        ...
  resources:
    image.png
    audio.wav
```

Translations will be done using fluent `.ftl` files. file structure tbd
