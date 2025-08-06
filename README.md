💰 Vault Stellar
Um contrato inteligente para a rede Stellar, desenvolvido em Rust com o Soroban SDK, que permite depósitos e saques de tokens, além de controle de saldo e administração segura.

⚙️ Funcionalidades
🆕 Inicialização: Define o proprietário e o token que será gerenciado.

💸 Depósito: Usuários podem depositar tokens no cofre.

🏧 Saque: Permite o saque dos tokens, com verificação de saldo.

📊 Consulta de Saldo: Qualquer usuário pode consultar seu saldo interno.

📦 Consulta de Suprimento Total: Mostra o total de tokens armazenados.

👤 Consulta de Proprietário e Token: Exibe quem é o dono do contrato e qual token está vinculado.

🔍 Como Funciona
🧱 Inicialização
O proprietário chama initialize, informando seu endereço e o token. Essa função só pode ser chamada uma vez.

➕ Depósito
O usuário chama deposit, informa o valor, e o contrato transfere os tokens para si, atualizando o saldo interno.

➖ Saque
O usuário chama withdraw e informa o valor desejado. Se o saldo for suficiente, os tokens são devolvidos.

📥 Consultas
balance → consulta de saldo do usuário

total_supply → total depositado no cofre

owner → endereço do proprietário

token_address → token vinculado ao contrato

🧩 Estrutura do Código
🧱 Vault: Estrutura principal do contrato

🗝️ DataKey: Enum para chaves de armazenamento (saldo, proprietário, etc.)

❗ VaultError: Enum para erros comuns (não autorizado, saldo insuficiente, etc.)

🧪 Exemplo de Uso
A lógica principal está no arquivo:

bash
Copiar
Editar
lib.rs
🛠️ Requisitos
🦀 Rust

🌟 Soroban SDK

🧰 Como Compilar
bash
Copiar
Editar
# Instale o Rust e o Soroban CLI
# Clone este repositório
# Compile com:
soroban build
📄 Licença
MIT
