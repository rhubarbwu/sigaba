KRYPTOS=KRYPTOSABCDEFGHIJLMNQUVWXZ
K1_PLAIN=$(./target/debug/sigaba vigenere -d -a $KRYPTOS -k PALIMPSEST -I src/kryptos/k1_cipher.txt)
echo "=== K1 Plain ==="
echo "$K1_PLAIN"

K2_PLAIN=$(./target/debug/sigaba vigenere -d -a $KRYPTOS -k ABSCISSA -I src/kryptos/k2_cipher.txt)
echo "=== K2 Plain ==="
echo "$K2_PLAIN"

K3_HALFWAY=$(./target/debug/sigaba rotate -d -a $KRYPTOS -n 8 -I src/kryptos/k3_cipher.txt)
K3_PLAIN=$(./target/debug/sigaba rotate -d -a $KRYPTOS -n 24 -i "$K3_HALFWAY")
echo "=== K3 Plain ==="
echo "$K3_PLAIN"
