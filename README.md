Vault Stellar
Um contrato inteligente para a rede Stellar, desenvolvido em Rust usando o Soroban SDK, que permite depósitos e saques de tokens, além de controle de saldo e administração.
Funcionalidades
Inicialização: Define o proprietário e o endereço do token que será gerenciado pelo cofre.
Depósito: Usuários podem depositar tokens no cofre, aumentando seu saldo interno.
Saque: Usuários podem sacar tokens, reduzindo seu saldo interno.
Consulta de Saldo: Qualquer usuário pode consultar seu saldo armazenado no cofre.
Consulta de Suprimento Total: Permite verificar o total de tokens depositados no cofre.
Consulta de Proprietário e Token: Permite verificar quem é o proprietário do contrato e qual token está sendo gerenciado.
Como funciona
Inicialização
O proprietário chama a função initialize, informando seu endereço e o endereço do token.
O contrato só pode ser inicializado uma vez.
Depósito
O usuário chama a função deposit, informando o valor a ser depositado.
O contrato transfere os tokens do usuário para si mesmo e atualiza o saldo interno.
Saque
O usuário chama a função withdraw, informando o valor a ser sacado.
O contrato transfere os tokens de volta para o usuário, desde que ele tenha saldo suficiente.
Consultas
Funções para consultar saldo (balance), suprimento total (total_supply), proprietário (owner) e endereço do token (token_address).
Estrutura do Código
Vault: Estrutura principal do contrato.
DataKey: Enum para chaves de armazenamento (saldo, proprietário, token, suprimento total).
VaultError: Enum para erros comuns (não autorizado, saldo insuficiente, valor inválido, não inicializado).
Exemplo de Uso
Apply to lib.rs
Requisitos
Rust
Soroban SDK
Como compilar
Instale o Rust e o Soroban CLI.
Clone este repositório.
Compile o contrato:
Apply to lib.rs
Licença
MIT
