import React from "react";
import { RouteLayout } from "./Layout.js";
import { Form, FormControl, Alert } from "react-bootstrap";
import Loading from "./Loading.js";
import { CommandGroup } from "shared-ui";
import commands from 'toml-loader!../../../shared/commands.toml';

function hash(s) {
  let out = new Set();

  for (let e of s.split(/\s+/)) {
    e = e.toLowerCase().replace(/[\s!<>`]+/, '');

    if (e.length === 0) {
      continue;
    }

    out.add(e);
  }

  return out;
}

function matches(test, s) {
  s = hash(s);

  for (let value of test.values()) {
    if (!setAny(s.values(), s => s.startsWith(value))) {
      return false;
    }
  }

  return true;

  function setAny(values, f) {
    for (let value of values) {
      if (f(value)) {
        return true;
      }
    }

    return false;
  }
}

export default class Help extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      loading: true,
      error: null,
      groups: commands.groups,
      filter: "",
    }
  }

  componentDidMount() {
    this.setState({loading: false});
  }

  filter(groups) {
    if (this.state.filter === "") {
      return groups;
    }

    let test = hash(this.state.filter);

    groups = groups.map(g => {
      let commands = g.commands.filter(c => {
        return matches(test, c.name);
      });

      let modified = commands.length != g.commands;
      return Object.assign({}, g, {commands, modified});
    });

    return groups.filter(g => g.commands.length > 0);
  }

  handleOnChange(e) {
    this.setState({filter: e.target.value});
  }

  render() {
    let error = null;

    let groups = this.filter(this.state.groups);

    return (
      <RouteLayout>
        <h2 className="page-title">Command Help</h2>

        <Alert variant="info">
          <b>Would you like to help expand this page?</b><br />

          Please contribute to the <a href="https://github.com/udoprog/OxidizeBot/blob/master/shared/commands.toml"><code>commands.toml</code></a> file that this is based off!
        </Alert>

        <Form>
          <FormControl value={this.state.filter || ""} onChange={e => this.handleOnChange(e)} />
        </Form>

        <Loading isLoading={this.state.loading} />
        {error}

        {groups.map(c => {
          return <CommandGroup key={c.name} {...c} />;
        })}
      </RouteLayout>
    );
  }
}