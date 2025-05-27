#!/bin/bash

echo "🚀 Сборка ядра..."
cargo bootimage
if [ $? -ne 0 ]; then
    echo "❌ Ошибка сборки!"
    exit 1
fi

echo "📁 Проверяем бинарник..."
if [ ! -f "target/x86_64-unknown-none/debug/bootimage-kernel_os.bin" ]; then
    echo "❌ Бинарник не найден!"
    exit 1
fi

echo "💿 Создаем загрузочный ISO образ..."
mkdir -p iso_temp
cp target/x86_64-unknown-none/debug/bootimage-kernel_os.bin iso_temp/bootimage.bin

# Создаем загрузочный ISO для VMware
hdiutil create -o kernel_os.dmg -volname "KernelOS" -srcfolder iso_temp
hdiutil convert kernel_os.dmg -format UDTO -o kernel_os.iso
mv kernel_os.iso.cdr kernel_os.iso

if [ $? -eq 0 ]; then
    echo "✅ Загрузочный ISO образ создан: kernel_os.iso"
else
    echo "❌ Ошибка создания ISO!"
fi

echo "🧹 Очищаем временные файлы..."
rm -rf iso_temp kernel_os.dmg

echo "🎉 Готово! Загружайте kernel_os.iso в VMware"