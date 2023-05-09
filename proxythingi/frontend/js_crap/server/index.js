"use strict";
Object.defineProperty(exports, "__esModule", {value: true});
const Koa = require("koa");
const serve = require("koa-static");
const cors = require("@koa/cors");
const path = require("path");
const dotenv = require("dotenv");
dotenv.config();
const port = process.env.PORT || 5000;
const app = new Koa();
app.use(cors());
const fe_app_dir = '../feapp/dist';
const fe_app_path = path.join(__dirname, fe_app_dir);
app.use(serve(fe_app_path));
console.log("frontend: " + fe_app_path + " on port: " + port);
app.listen(port);
//# sourceMappingURL=index.js.map