Add-Type -AssemblyName System.Drawing

# Create a 256x256 bitmap
$bmp = New-Object System.Drawing.Bitmap 256, 256
$g = [System.Drawing.Graphics]::FromImage($bmp)

# Fill with blue color
$g.Clear([System.Drawing.Color]::Blue)

# Clean up graphics
$g.Dispose()

# Save as ICO file
$bmp.Save('icons\icon.ico', [System.Drawing.Imaging.ImageFormat]::Icon)

# Clean up bitmap
$bmp.Dispose()

Write-Host "Valid ICO file created successfully!" 