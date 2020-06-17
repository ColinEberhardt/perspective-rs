import Table from "./table";
import { schema, data } from "./olympics";

const view_handler = {
  get: function (target, prop, receiver) {
    console.log("view", prop);
    if (prop === "to_columns") {
      const t0 = performance.now();
      target.to_columns().then(c => {
        const t1 = performance.now();
        console.log(`Call to to_columns took ${t1 - t0} milliseconds.`);
      });
    }
    if (prop === "num_rows") {
      console.log(target.num_rows());
    }
    if (prop === "column_paths") {
      console.log(target.column_paths());
    }
    if (typeof prop === "function") {
      console.log("view", prop, prop());
    }
    return Reflect.get(...arguments);
  }
};

const table_handler = {
  get: function (target, prop, receiver) {
    console.log("table", prop);
    if (prop === "view") {
      return function (...args) {
        const view = target.view(...args);
        console.log(args[0]);
        const view_proxy = new Proxy(view, view_handler);
        return view_proxy;
      };
    } else {
      return Reflect.get(...arguments);
    }
  }
};

const stuff = [
  { x: 1, y: "a", z: "fish" },
  { x: 3, y: "a", z: "fish" },
  { x: 1, y: "c", z: "fish" },
  { x: 2, y: "a", z: "cat" },
  { x: 4, y: "a", z: "cat" },
  { x: 3, y: "b", z: "cat" },
  { x: 5, y: "c", z: "cat" },
  { x: 5, y: "d", z: "cat" },
  { x: 5, y: "d", z: "cat" }
];

const stuff_schema = {
  x: "integer",
  y: "string",
  z: "string"
};

const table = new Table(schema, data);
viewer.toggleConfig();
viewer.load(new Proxy(table, table_handler));

// const worker = perspective.worker();
// const table = worker.table(schema);
// table.update(data);

// viewer.toggleConfig();
// viewer.load(new Proxy(table, table_handler));
