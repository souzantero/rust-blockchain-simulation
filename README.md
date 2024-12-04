# Simulador de Blockchain em Rust

Este projeto é uma simulação de uma blockchain implementada em Rust. A biblioteca permite criar uma instância de uma blockchain, adicionar transações, validar sua integridade e corromper a cadeia para fins de teste.

## Funcionalidades

- **Criar Transações**: Adicione transações especificando a origem, o destino e o valor.
- **Agrupamento em Blocos**: As transações são adicionadas a blocos. Ao atingir o número máximo de transações por bloco, um novo bloco é criado.
- **Validação de Integridade**: Verifique se todos os blocos estão sequenciados corretamente, cada um contendo o hash adequado e o hash do bloco anterior.
- **Corrupção da Blockchain**: Corrompa a cadeia de blocos intencionalmente para fins de teste.

## Pré-requisitos

- **Rust**: Certifique-se de ter o Rust instalado em sua máquina. Você pode instalá-lo a partir do site oficial do Rust.

## Instalação

Clone o repositório do projeto e navegue até o diretório:

```bash
git clone <URL_DO_REPOSITÓRIO>
cd <NOME_DO_DIRETÓRIO>
```

## Como Executar

### Compilar o Projeto

Execute o seguinte comando para compilar a aplicação:

```
cargo build
```

### Executar a Aplicação

Para executar a aplicação principal:

```
cargo run
```

### Executar os Testes

Para executar os testes automatizados:

```
cargo test
```

## Uso da Aplicação

Ao executar o programa, um menu interativo será exibido no terminal:

1.	Criar Transação: Insira a origem, o destino e o valor para adicionar uma nova transação à blockchain.
2.	Verificar Integridade: Verifique a integridade da blockchain atual.
3.	Corromper Blockchain: Corrompa a blockchain para fins de teste.
4.	Sair: Encerre a aplicação.

## Estrutura do Projeto

-	**src/main.rs**: Arquivo principal da aplicação que gerencia a interface de linha de comando.
- **src/lib.rs**: Contém a biblioteca com a implementação da blockchain.
