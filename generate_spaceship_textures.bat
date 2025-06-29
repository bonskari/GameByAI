@echo off
echo Generating high-quality spaceship textures...

echo.
echo Generating tech panel texture...
python tools\sdxl_spaceship_generator.py single "futuristic spaceship tech panel, metallic blue surface, glowing circuits, clean geometric design, brushed metal finish" assets\textures\tech_panel_new.png

echo.
echo Generating hull plating texture...
python tools\sdxl_spaceship_generator.py single "spaceship hull plating, weathered grey metal plates, rivets and bolts, industrial design, worn steel panels, space vessel exterior" assets\textures\hull_plating_new.png

echo.
echo Generating control system texture...
python tools\sdxl_spaceship_generator.py single "spaceship control system interface, metallic orange accent panels, digital displays, buttons and switches, command center design" assets\textures\control_system_new.png

echo.
echo Generating energy conduit texture...
python tools\sdxl_spaceship_generator.py single "spaceship energy conduit, metallic green power channels, glowing energy lines, technical patterns, power distribution system" assets\textures\energy_conduit_new.png

echo.
echo Generating floor texture...
python tools\sdxl_spaceship_generator.py single "spaceship floor plating, dark grey metal grating, anti-slip texture, industrial flooring, worn metal walkway" assets\textures\floor_new.png

echo.
echo Generating ceiling texture...
python tools\sdxl_spaceship_generator.py single "spaceship ceiling panels, clean white metal tiles, ventilation grilles, overhead lighting strips, sterile interior design" assets\textures\ceiling_new.png

echo.
echo All spaceship textures generated!
echo New textures saved with "_new" suffix in assets/textures/
echo Update your JSON configuration to use the new textures.
pause 