# Análise de Regressão Linear Simples em Rust

Este projeto implementa uma análise de regressão linear simples em Rust. A regressão linear é uma técnica estatística utilizada para modelar a relação entre uma variável dependente e uma ou mais variáveis independentes.

## Funcionalidades

- Cálculo da regressão linear (coeficientes da reta).
- Cálculo das métricas de avaliação:
  - **R²** (coeficiente de determinação): mede a qualidade do ajuste do modelo.
  - **MSE** (Erro Quadrático Médio): avalia a precisão do modelo, representando a média dos erros quadráticos.
- Previsões baseadas no modelo treinado.
- Testes automatizados para garantir a qualidade do código.

## Como Executar

Para rodar o projeto e ver a análise em ação, utilize o comando:

```bash
cargo run
```

Isso irá calcular a regressão para um conjunto de dados simples e mostrar o modelo ajustado, juntamente com as métricas R² e MSE, além das previsões para novos dados.

### Exemplo de Saída:

```bash
y = 2.00x + 0.00
R² = 1.0000
MSE = 0.0000
Previsões: [12.0, 14.0]
```

## Como Testar

Para rodar os testes automatizados, use o comando:

```bash
cargo test
```

Todos os testes, incluindo cálculos de regressão, R², MSE e previsões, são realizados e validados para garantir que o código esteja funcionando corretamente.


## Como Contribuir

Se você quiser contribuir para este projeto, faça um fork, crie uma branch com suas mudanças e envie um pull request. Todos são bem-vindos!

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo LICENSE para mais detalhes.