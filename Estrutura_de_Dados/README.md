# Tipos Abstratos de Dados (TAD)

Tipos abstratos de dados são modelos de matemáticos que definem operações sobre os dados sem definir a implementação. Assim podemos reutilizar o código em diferentes aplicações. Um exemplo seria a implementação de um cadastro de funcionários, para isso teriamos que definir os **dados de interesse**:

- Nome
- Cargo
- Salário

E as **operações** que poderiam ser realizadas sobre esses dados seriam:

- Inserir um novo funcionário
- Remover um funcionário
- Alterar o salário de um funcionário
- Alterar o cargo de um funcionário

Um TAD tem como características:

- **Ocultação** da implemetação das operações, assim o usuário só tem conhecimento das funcionalidades.
- **Encapsulamento** dos dados e operações.
- Os dados só podem ser acessados através das operações definidas.

Utilizando TAD o programa fica mais fácil de implementar e mais seguro, pois os dados só são acessados mediante as operações. O código também ganha maior portabilidade, maior independência e sua manutenção é mais fácil. E, claro, fica mais fácil de reutilizar.
