Quero que você crie um app para a área de enfermagem com foco principal em follow-up de atendimentos realizados e futuros.

Objetivo do app:
Desenvolver um sistema simples, organizado e fácil de usar para registrar atendimentos, acompanhar retornos e visualizar próximos follow-ups de pacientes.

Stack obrigatória:
- Linguagem: Rust
- Framework principal: Dioxus
- Arquitetura: organizada, modular e pronta para evolução
- Código limpo, idiomático e bem estruturado
- Interface clara, responsiva e voltada para produtividade
- Banco de dados local para MVP: SQLite
- Preparar o projeto para futura expansão

Antes de começar a implementar:
1. Gere um plano de implementação completo
2. Defina a arquitetura do projeto
3. Liste as entidades e campos
4. Sugira a estrutura de pastas
5. Descreva rapidamente o fluxo de navegação do app
6. Só depois comece a gerar o código

Escopo funcional do app:

Entidade principal: Atendimento / Follow-up

Campos obrigatórios:
- paciente
- tipo de atendimento
- data atendimento
- próximo atendimento
- observação

Campos adicionais recomendados:
- id
- status do follow-up (pendente, realizado, atrasado, cancelado)
- profissional responsável
- telefone ou contato do paciente
- canal do atendimento (presencial, telefone, WhatsApp, vídeo)
- prioridade
- criado em
- atualizado em

Regras de negócio:
- O usuário deve conseguir cadastrar um atendimento realizado
- O usuário deve conseguir definir a data do próximo atendimento
- O usuário deve conseguir adicionar observações livres
- O usuário deve conseguir editar e excluir registros
- O sistema deve destacar follow-ups vencidos
- O sistema deve destacar follow-ups que acontecerão hoje
- O sistema deve permitir filtrar por paciente, tipo de atendimento, status e período
- O sistema deve validar campos obrigatórios
- O próximo atendimento não pode ser anterior à data do atendimento, salvo se o usuário confirmar explicitamente uma exceção
- Toda alteração deve atualizar o campo "atualizado em"

Telas do app:
1. Dashboard
- resumo de atendimentos do dia
- próximos follow-ups
- follow-ups em atraso
- indicadores rápidos

2. Lista de atendimentos
- tabela ou lista com busca
- filtros por paciente, tipo, status e data
- ordenação por próximo atendimento

3. Novo atendimento
- formulário completo
- validação
- botão salvar e salvar + novo

4. Detalhe do atendimento
- visualizar histórico
- editar dados
- registrar nova observação
- reagendar próximo atendimento

5. Agenda / follow-ups futuros
- visão por data
- foco nos retornos agendados
- destaque visual para urgências e atrasos

Requisitos de UX:
- interface simples e objetiva
- formulários rápidos
- navegação intuitiva
- feedback visual ao salvar, editar e excluir
- cores discretas e aparência profissional
- acessibilidade básica
- layout adequado para desktop e tablet

Requisitos técnicos:
- usar componentes reutilizáveis
- separar bem camadas de UI, estado e persistência
- criar modelos e serviços de forma clara
- evitar acoplamento excessivo
- organizar o código para manutenção futura
- incluir tratamento de erros
- incluir dados de exemplo para facilitar testes
- incluir comentários apenas quando realmente agregarem valor

Funcionalidades extras desejáveis:
- campo de histórico de follow-up por paciente
- alerta visual para retornos atrasados
- busca rápida por nome do paciente
- duplicar atendimento para agilizar cadastro
- exportação simples em CSV no futuro, deixando o código preparado
- confirmação antes de excluir
- indicador de último contato realizado

Persistência:
- usar SQLite no MVP
- criar schema inicial
- preparar migrations ou estrutura equivalente, se fizer sentido
- criar repositório/local storage de forma fácil de evoluir depois

Qualidade:
- gerar código funcional
- evitar pseudocódigo
- incluir instruções de execução do projeto
- incluir um README objetivo
- criar testes básicos para regras essenciais
- ao final, apresentar um walkthrough do que foi implementado

Entregáveis esperados:
- plano de implementação
- estrutura do projeto
- código completo
- instruções para rodar
- breve walkthrough final
