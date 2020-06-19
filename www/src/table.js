import * as wasm from "perspective-rs";

import computedFunctions from "./computed-functions";
import View from "./view";

export default class Table {
  constructor(schema, data) {
    this._data = data;
    this._adaptee = new wasm.Table(data);
    this._schema = schema;
    this.type = "table";
  }

  columns() {
    return Promise.resolve(this._adaptee.columns().split(","));
  }

  schema() {
    return Promise.resolve(this._schema);
  }

  computed_schema() {
    return Promise.resolve({});
  }

  compute() {
    return "";
  }

  make_port() {
    return "";
  }

  view(config) {
    config = {
      columns: [],
      aggregates: [],
      column_pivots: [],
      computed_columns: [],
      filter: config.filter
        ? config.filter
            .map(filter => {
              if (filter[1] === "==") {
                filter[1] = "eqeq";
              }
            })
        : [],
      row_pivots: [],
      sort: [],
      ...config
    };
    config.filter = config.filter.filter(f => f[2] !== null)
    const view = this._adaptee.to_view(JSON.stringify(config));
    return new View(config, view, this);
  }

  get_computed_functions() {
    return computedFunctions;
  }

  size() {
    return this._adaptee.size();
  }

  is_valid_filter() {
    return Promise.resolve(true);
  }
}
