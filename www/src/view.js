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

  to_columns(options) {
    options.end_col = Math.ceil(options.end_col);
    options.end_row = Math.ceil(options.end_row);
    options.start_col = Math.ceil(options.start_col);
    options.start_row = Math.ceil(options.start_row);
    const cols = this._adaptee.to_columns(JSON.stringify(options));
    const ret = {
      ...cols.columns,
      __ROW_PATH__: cols.row_paths.map(s => s.values)
    };
    return Promise.resolve(ret);
  }

  to_json() {
    const json = this._adaptee.to_json();
    let res = [];
    json.rows.forEach((row, index) => {
      res.push({
        ...row,
        __ROW_PATH__: json.row_paths[index].values
      });
    });
    return Promise.resolve(res);
  }

  column_paths() {
    // TODO - sort our column ordering - we sort here to push __ROW_PATH__ to the front
    const paths = this._adaptee.columns().split(",");
    return Promise.resolve(paths);
  }
}
