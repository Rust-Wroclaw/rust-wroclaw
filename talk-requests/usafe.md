# Talk o pisaniu unsafe kodu
Czasem mówi się, że kod w Ruscie w ogóle nie potrzebuje unsafe,
niekiedy z taką stanowczością, że wydawałoby się, że jest to
feature który mógłby nie istnieć w publicznym API języka.

Z drugiej strony są głosy, że u`unsafe` wcale nie jest sam w sobie
niebezpieczny, moze być tylko źle użyty.

Jeszcze inni zauważają, że `unsafe` powoduje degradację Rusta
do języków typu C, bo można za jego pomocą wyłączyć wszelkie gwarancje
języka.

Jak to z tym jest? Przydała by się prelekcja o tym czym tak na prawdę jest
`unsafe`, jakie są jego dobre i złe strony, najlepiej z przykładami "dobrego"
użycia, być może zostawiającego z jakimiś guidelinami.

## Materialy
Warto byłoby oprzeć się o [Rustonomicon](https://doc.rust-lang.org/nomicon/)
i [Unsafe guidelines](https://github.com/rust-lang/unsafe-code-guidelines)
