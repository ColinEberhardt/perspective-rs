import Table from "./table";
import {schema, data} from "./olympics";


const stuff = [
  { x: 1, y: "a", z: true },
  { x: 1, y: "b", z: false },
  { x: 1, y: "c", z: true },
  { x: 4, y: "d", z: false },
  { x: 4, y: "d", z: false },
  { x: 3, y: "e", z: false },
  { x: 5, y: "a", z: false },
  { x: 5, y: "b", z: false },
  { x: 5, y: "f", z: false },
  { x: 1, y: "a", z: true },
  { x: 1, y: "b", z: false },
  { x: 1, y: "c", z: true },
  { x: 4, y: "d", z: false },
  { x: 4, y: "d", z: false },
  { x: 3, y: "e", z: false },
  { x: 5, y: "a", z: false },
  { x: 5, y: "b", z: false },
  { x: 5, y: "f", z: false },
];

const stuff_schema = {
  x: "integer",
  y: "string",
  z: "boolean"
};

const table = new Table(stuff_schema, stuff);
viewer.toggleConfig();
viewer.load(table);

// const worker = perspective.worker();
// const table = worker.table(schema);
// table.update(data);

// const view_handler = {
//   get: function (target, prop, receiver) {
//     console.log("view", prop);
//     if (prop === "to_columns") {
//       target.to_columns().then(c => {
//         console.log(JSON.stringify(c))
//       })
//     }
//     if (prop === "num_rows") {
//       console.log(target.num_rows())
//     }
//     if (prop === "column_paths") {
//       console.log(target.column_paths())
//     }
//     if (typeof prop === "function") {
//       console.log("view", prop, prop());
//     }
//     return Reflect.get(...arguments);
//   }
// };

// const table_handler = {
//   get: function (target, prop, receiver) {
//     console.log("table", prop);
//     if (prop === "view") {
//       return function (...args) {
//         const view = target.view(...args);
//         console.log(args[0]);
//         const view_proxy = new Proxy(view, view_handler)
//         return view_proxy;
//       };
//     } else {
//       return Reflect.get(...arguments);
//     }
//   }
// };

// viewer.toggleConfig();
// viewer.load(table);


