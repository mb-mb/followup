---
trigger: always_on
---

# Clean Architecture - Rust + Dioxus + SQLite

Este projeto deve seguir Clean Architecture de forma rígida.

Objetivo:
Construir um app de enfermagem focado em follow-up de atendimentos com arquitetura limpa, modular, testável e preparada para evolução.

## Diretrizes obrigatórias

- Usar Rust como linguagem principal.
- Usar Dioxus como framework principal para a camada de apresentação.
- Usar SQLite como persistência do MVP.
- Priorizar separação clara entre regras de negócio, casos de uso, interface e infraestrutura.
- Toda decisão arquitetural deve privilegiar baixo acoplamento, alta coesão, testabilidade e manutenção.

## Camadas obrigatórias

O projeto deve ser organizado nas seguintes camadas:

1. Domain
- Contém entidades, value objects, enums de negócio, regras de negócio puras e contratos centrais.
- Não pode depender de Dioxus, SQLite, sqlx, Axum, HTTP, UI ou detalhes de infraestrutura.
- Deve ser a camada mais estável do sistema.

2. Application
- Contém casos de uso, serviços de aplicação, DTOs de entrada e saída, validações de fluxo e orquestração.
- Pode depender da camada Domain.
- Não pode depender diretamente de componentes de UI nem de implementações concretas de banco.
- Deve trabalhar com traits/interfaces para repositórios e gateways.

3. Infrastructure
- Contém implementações concretas de persistência, acesso SQLite, repositórios, configuração, migrações e integrações externas.
- Pode depender de Domain e Application para implementar contratos.
- Toda lógica de banco deve ficar aqui.
- Preferir sqlx para acesso ao SQLite.
- Evitar espalhar SQL fora desta camada.

4. Presentation
- Contém telas, componentes, rotas, estado de interface, formulários e experiência do usuário em Dioxus.
- Pode depender de Application.
- Não pode acessar SQLite diretamente.
- Não deve conter regra de negócio crítica.
- Toda ação relevante deve chamar um caso de uso da camada Application.

## Regra de dependências

As dependências devem sempre apontar para dentro:
- Presentation pode depender de Application
- Infrastructure pode depender de Application e Domain
- Application pode depender de Domain
- Domain não depende de nenhuma camada externa

Nunca permitir:
- Presentation -> SQLite direto
- Presentation -> sqlx direto
- Domain -> Dioxus
- Domain -> banco de dados
- Application -> componentes visuais
- Infrastructure ditando regra de negócio

## Padrões de implementação

- Usar traits para abstrair repositórios e serviços.
- Implementações concretas devem ficar na camada Infrastructure.
- Casos de uso devem ser explícitos e pequenos.
- Preferir structs e enums para modelagem de domínio.
- Evitar lógica de negócio em handlers, componentes visuais ou código de banco.
- Evitar funções gigantes.
- Cada módulo deve ter responsabilidade clara.
- Erros devem ser tipados e previsíveis.
- Priorizar composição em vez de acoplamento implícito.

## Estrutura sugerida

Sempre que possível, gerar o projeto com estrutura semelhante a esta:

```text
src/
  domain/
    entities/
    value_objects/
    enums/
    errors/
    repositories/
  application/
    dto/
    use_cases/
    services/
  infrastructure/
    db/
    repositories/
    migrations/
    config/
  presentation/
    components/
    pages/
    routes/
    state/
  lib.rs
  main.rs
