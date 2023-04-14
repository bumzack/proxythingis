import * as Koa from "koa";
import * as serve from "koa-static";
import * as cors from "@koa/cors";
import * as path from "path";
import * as dotenv from 'dotenv'

export {}

dotenv.config()

const port = process.env.PORT || 5000


const app = new Koa()
app.use(cors());
const fe_app_dir = '../fe/dist';
const fe_app_path = path.join(__dirname, fe_app_dir);
console.log("path ", fe_app_path);
app.use(serve(fe_app_path))

console.log("searchapp: " + fe_app_path + " on port: " + port);
app.listen(port)
