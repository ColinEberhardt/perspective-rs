export default class View {
  constructor(config, adaptee, table) {
    this._config = config;
    this._adaptee = adaptee;
    this._table = table;
  }

  get_config() {
    return this._config;
  }

  delete() {}

  on_update() {}

  remove_update() {}

  num_rows() {
    return Promise.resolve(this._adaptee.num_rows);
  }

  num_columns() {
    return Promise.resolve(this._adaptee.num_columns);
  }

  schema() {
    return this._table.schema();
  }

  to_columns() {
    const t0 = performance.now();
    let cols = this._adaptee.to_columns();
    const t1 = performance.now();
    if (cols.__ROW_PATH__) {
      cols.__ROW_PATH__ = cols.__ROW_PATH__.map(s => s ? [s] : []);
    }
    console.log(`Call to to_columns took ${t1 - t0} milliseconds.`);
    return Promise.resolve(cols);
  }

  column_paths() {
    // TODO - sort our column ordering - we sort here to push __ROW_PATH__ to the front
    const paths = this._adaptee.columns().split(",");
    return Promise.resolve(paths);
  }
}