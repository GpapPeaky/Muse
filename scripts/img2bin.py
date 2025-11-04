from PIL import Image
import os

# Define your icon sizes and paths
icon_sizes = [16, 32, 64]  # small, medium, big
input_folder = "assets/icon/"
output_folder = "assets/icon/"

for size in icon_sizes:
    input_path = os.path.join(input_folder, f"muse.bmp")
    output_path = os.path.join(output_folder, f"muse{size}.bin")

    # Open and resize the image (ensure correct size)
    img = Image.open(input_path).convert("RGBA")
    if img.size != (size, size):
        img = img.resize((size, size), Image.LANCZOS)

    # Get raw RGBA bytes
    data = img.tobytes()  # Returns bytes in RGBA order

    # Write to .bin
    with open(output_path, "wb") as f:
        f.write(data)

    print(f"Created {output_path} ({len(data)} bytes)")
