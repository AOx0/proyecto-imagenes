set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

run:
    tailwindcss -c tailwind.config.js -o public/assets/tailwind.css
    dioxus build
    ./dist/proyecto