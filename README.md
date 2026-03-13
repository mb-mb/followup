# 🏥 FollowUp — Nursing Follow-up App

App de enfermagem para gestão de atendimentos e acompanhamento de follow-ups, construído com **Rust**, **Dioxus 0.7** e **SQLite**.

## ✨ Funcionalidades

- **Dashboard** — Visão geral com indicadores (total, hoje, atrasados, pendentes, realizados)
- **Lista de Atendimentos** — Tabela com busca e filtros por status, prioridade e canal
- **Novo Atendimento** — Formulário completo com validação de campos obrigatórios
- **Detalhe/Edição** — Visualização, edição, exclusão com confirmação e atualização de status
- **Agenda** — Follow-ups agrupados por data com destaque para atrasados e urgentes

## 🏗️ Arquitetura

O projeto segue **Clean Architecture** com 4 camadas:

```
src/
├── domain/           # Entidades, enums, erros, traits de repositório
├── application/      # DTOs, validações, casos de uso (AttendanceService)
├── infrastructure/   # SQLite (rusqlite), repositório concreto, seed de dados
├── presentation/     # Dioxus: rotas, componentes, páginas, CSS
├── lib.rs
└── main.rs
```

| Regra | Cumprida |
|-------|----------|
| Domain sem dependências externas | ✅ |
| Application depende apenas de Domain | ✅ |
| Infrastructure implementa traits do Domain | ✅ |
| Presentation chama use cases da Application | ✅ |
| UI nunca acessa SQLite diretamente | ✅ |

## 🛠️ Stack

- **Rust** (edition 2021)
- **Dioxus 0.7** — framework de UI com router
- **rusqlite** (bundled) — persistência local SQLite
- **chrono** — manipulação de datas
- **uuid** — geração de IDs
- **serde** — serialização

## 🚀 Como Executar

### Pré-requisitos

- Rust (1.75+)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/) (opcional, para hot-reload)

### Rodar o app

```bash
cargo run
```

O app abre como uma janela desktop. O banco `followup.db` é criado automaticamente na primeira execução com dados de exemplo.

> **Dica:** Para resetar os dados, apague o arquivo `followup.db` e reinicie o app.

### Com hot-reload (via Dioxus CLI)

```bash
dx serve
```

## 📋 Campos do Atendimento

| Campo | Obrigatório |
|-------|:-----------:|
| Paciente | ✅ |
| Tipo de atendimento | ✅ |
| Data do atendimento | ✅ |
| Próximo atendimento | ✅ |
| Profissional responsável | ✅ |
| Canal (Presencial, Telefone, WhatsApp, Vídeo) | — |
| Prioridade (Baixa, Média, Alta, Urgente) | — |
| Status (Pendente, Realizado, Atrasado, Cancelado) | — |
| Observações | — |

## 📂 Targets Disponíveis

```bash
cargo run                          # Desktop (padrão)
cargo run --features web           # Web
cargo run --features mobile        # Mobile
```

## 📄 Licença

Projeto privado.
