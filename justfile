set shell := ["powershell.exe","-c"]

run:
    tailwindcss -c tailwind.config.js -o public/assets/tailwind.css
    dioxus build
    ./dist/proyecto