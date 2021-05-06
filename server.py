import aiohttp
import asyncio
from fastapi import FastAPI, Request
from time import time
from fastapi.templating import Jinja2Templates

app = FastAPI()
app.last_refresh = 0
app.scoreboard = []


users = {
        "shyba": 770,
        "uncanned": 0,
        "innng": 3,
        "consoli": 2,
        "luizdepra": 65,
        "marcospb19": 12,
        "v0idpwn": 2
}
refresh_every = 60
templates = Jinja2Templates(directory="templates")


async def fetch_state():
    if (time() - app.last_refresh) > refresh_every:
        app.last_refresh = time()
        await asyncio.ensure_future(refresh_task())
    return app.scoreboard

async def refresh_task():
    scoreboard = []
    async with aiohttp.ClientSession() as session:
        for user, initial_points in users.items():
            url = f"https://www.codewars.com/api/v1/users/{user}"
            async with session.get(url) as response:
                response = await response.json()
                print(response)
                current_points = response["honor"]
                scoreboard.append((current_points - initial_points, current_points, initial_points, user))
    scoreboard.sort(reverse=True)
    app.scoreboard = scoreboard


@app.get("/")
async def root(request: Request):
    state = await fetch_state()
    return templates.TemplateResponse("index.html", {"request": request, "score": state})
