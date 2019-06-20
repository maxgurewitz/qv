import AppBar from '@material-ui/core/AppBar';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';
import Button from '@material-ui/core/Button';
import Toolbar from '@material-ui/core/Toolbar';
import React from 'react';
import { Dispatch } from 'redux';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Action, CombinedState } from './types';
import styles from './LoggedIn.module.css';

const LoggedIn: React.FC<LoggedInProps> = ({logOut}) => {
  return (
    <div className={styles.grow}>
      <AppBar position="static">
        <Toolbar>
          <Tabs value={0} onChange={console.log}>
            <Tab label="Item One" />
            <Tab label="Item Two" />
            <Tab label="Item Three" />
          </Tabs>
          <div className={styles.grow} />
          <Button onClick={logOut} variant="contained" color="secondary" >
              Log Out
          </Button>
        </Toolbar>
      </AppBar>
      <div> Logged in </div>
    </div>
  );
};

interface LoggedInStateProps {
}

type LoggedInProps = LoggedInStateProps & LoggedInDispatchProps;

const mapStateToProps: MapStateToPropsParam<LoggedInStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
  };
};

interface LoggedInDispatchProps {
  logOut: () => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    logOut: () => {
      dispatch({
        source: 'internal',
        type: 'LogOut',
      });
    }
  };
}

export default connect<LoggedInStateProps, LoggedInDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(LoggedIn);