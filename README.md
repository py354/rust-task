## Запуск в докере
Сервис использует порт 8080
```bash
docker build -t rust-task .
docker run -p 8080:8080 rust-task
```
## Endpoints
GET ```/balacnes``` возвращает таблицу с балансами пользователей:
```python
>>> requests.get('http://127.0.0.1:8080/balances').content
{"table": {"User1":0, "User2":1000, "User3":-100}}
```
POST ```/update``` обновляет балансы для указанных пользователей
```python
>>> requests.post('http://127.0.0.1:8080/update', json={'table': {'User1': 10000, 'User400': 400}})
```
POST ```/delete``` удаляет указанных пользователей
```python
>>> rerequests.post('http://127.0.0.1:8080/delete', json={'users': ['User2']})
```