const app = require("express")();
const sqlite = require("sqlite");
const Promise = require("bluebird");

const dbPromise = sqlite.open("../my_db.sqlite", { Promise });

app.get("/", async (req, res) => {
  const db = await dbPromise;
  const posts = await db.all("SELECT * FROM posts");

  res.send(posts);
});

const PORT = 3000;
app.listen(PORT, () => console.log(`Listening on port ${PORT}`));
