Vault Stellar
Um contrato inteligente desenvolvido em Rust utilizando o Soroban SDK, para a rede Stellar, que permite depósitos e saques de tokens, além do controle de saldo e administração.

Funcionalidades
Inicialização: Define o proprietário e o endereço do token a ser gerenciado pelo cofre.

Depósito: Usuários podem depositar tokens, aumentando seu saldo interno.

Saque: Usuários podem sacar tokens, reduzindo seu saldo interno.

Consulta de Saldo: Permite a qualquer usuário verificar seu saldo armazenado.

Consulta de Suprimento Total: Informa o total de tokens depositados no cofre.

Consulta de Proprietário e Token: Exibe o endereço do proprietário e o token vinculado ao contrato.

Como funciona
1. Inicialização
O proprietário chama a função initialize, informando seu endereço e o do token. A inicialização só pode ser feita uma vez.

2. Depósito
O usuário chama a função deposit, informando o valor. O contrato transfere os tokens para si mesmo e atualiza o saldo interno do usuário.

3. Saque
O usuário chama a função withdraw, com o valor desejado. O contrato verifica o saldo e, se suficiente, transfere os tokens de volta para o usuário.

4. Consultas
Funções disponíveis:

balance: consulta de saldo individual

total_supply: total de tokens no cofre

owner: endereço do proprietário

token_address: endereço do token gerenciado

Estrutura do Código
Vault: Estrutura principal do contrato.

DataKey: Enum para chaves de armazenamento (ex: saldo, proprietário, token, suprimento).

VaultError: Enum com erros comuns (não autorizado, saldo insuficiente, valor inválido, não inicializado).

Exemplo de Uso
A lógica do contrato deve ser aplicada no arquivo lib.rs.

Requisitos
Rust

Soroban SDK

Como compilar
Instale o Rust e o Soroban CLI.

Clone este repositório.

Compile o contrato com:

bash
Copiar
Editar
soroban build
Licença
MIT

