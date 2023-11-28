NUMERO=$1
NOME=$2
PASTA="$NUMERO - $NOME"

mkdir "$PASTA" && touch "$PASTA"/README.md && 
echo "# $NOME" >> "$PASTA"/README.md && echo "Criado com sucesso!"
