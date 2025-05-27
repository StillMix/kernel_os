#!/bin/bash

echo "Сборка ядра..."
cargo build

echo "Создание ISO структуры..."
mkdir -p iso/boot/grub

echo "Копирование ядра..."
cp target/x86_64-unknown-none/debug/kernel_os iso/boot/

echo "Создание конфига GRUB..."
cat > iso/boot/grub/grub.cfg << EOF
set timeout=0
set default=0

menuentry "Kernel OS" {
    multiboot /boot/kernel_os
    boot
}
EOF

echo "Поиск grub-mkrescue..."
if command -v x86_64-elf-grub-mkrescue &> /dev/null; then
    GRUB_MKRESCUE="x86_64-elf-grub-mkrescue"
elif command -v grub-mkrescue &> /dev/null; then
    GRUB_MKRESCUE="grub-mkrescue"
elif command -v grub2-mkrescue &> /dev/null; then
    GRUB_MKRESCUE="grub2-mkrescue"
else
    echo "grub-mkrescue не найден!"
    echo "Установите: brew install x86_64-elf-grub xorriso"
    exit 1
fi

echo "Создание ISO образа с помощью $GRUB_MKRESCUE..."
$GRUB_MKRESCUE -o kernel_os.iso iso

if [ ! -f kernel_os.iso ]; then
    echo "Ошибка: ISO не создался!"
    exit 1
fi

echo "Запуск QEMU..."
qemu-system-x86_64 -cdrom kernel_os.iso

echo "Очистка временных файлов..."
rm -rf iso kernel_os.iso