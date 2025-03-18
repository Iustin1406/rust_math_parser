# Rust Math Parser

## Descriere
Acest proiect este un **parser de expresii matematice** scris în **Rust**, care suportă operații aritmetice de bază, funcții matematice și puteri. Utilizează **Shunting Yard Algorithm** pentru a converti expresia într-o **forma poloneză inversată (RPN)** și construiește un **AST (Abstract Syntax Tree)** pentru evaluare.

## Caracteristici
- Suportă operatorii: `+`, `-`, `*`, `/`, `^^` (exponentiere)
- Funcții matematice disponibile: `sin`, `cos`, `sqrt`, `log`
- Conversie a expresiilor în **RPN**
- Construirea și evaluarea unui **AST**
  
## 🔍 Cum funcționează
- Tokenizare: Expresia este descompusă în numere, operatori, funcții și paranteze.
-	Conversie în RPN: Se utilizează Shunting Yard Algorithm pentru a obține o expresie postfixată.
-	Construire AST: Se creează un Abstract Syntax Tree din RPN.
-	Evaluare AST: Expresia este evaluată pas cu pas, afișând fiecare calcul intermediar.

## Utilizare:
```bash
cargo run -- <expresie>
```
