# safety_rusts (PT-BR)
Uma aplicação de console em rust para vasculhar arquivos, buscando possíveis scripts maliciosos

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
