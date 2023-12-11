# safety_rusts (PT-BR)
Uma aplicação de console em rust para vasculhar arquivos, buscando possíveis scripts maliciosos  
Essa aplicacao foi feita com o intuito de agilizar a localizacao de scripts maliciosos na pasta de cada usuario, o programa nao faz a varredura de todo o computador (principalmente por uma questao de processamento), somente na pasta home de cada usuario.  
Por hora sao buscadas linhas que contenham rm e sudo, mas caso sejam identificadas mais comandos que podem danificar ou prejudicar o sistema eles serao adicionados a lista  
O programa por hora, por reconhecer apenas comandos de linux nao tem um bom funcionamento no Windows, mas serao adicionados comandos de bat e outros scripts usados no Windows, e o programa sabera qual o sistema operacional utilizado, rechonhecendo quais comandos procurar

## Crates usadas

- log (0.4.20)
- log4rs (1.2.0)
	> log e log4rs usadas em conjunto para gerar os logs de quais linhas e arquivos contem o padrao especificado
- homedir (0.2.1)
	> homedir usada para dinamizar a busca dos arquivos dentro da pasta do usuario especifico, sem buscar nas pastas de outros usuarios que podem  
	estar usando a maquina

# safety_rusts (EN)
A rust console app that scans files, searching for possible harmful scripts

## Used Crates

- log (0.4.20)
- log4rs (1.2.0)
	> log and log4rs used together to generate logs of which lines and files contained the specified pattern
- homedir (0.2.1)
	> homedir used to find out dynamically the home folder of the current user, and avoid searching the files of other users that may be using  
	the same machine
