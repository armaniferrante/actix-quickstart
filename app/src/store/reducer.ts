export default function reducer(
  state: State = initialState,
  action: Action,
): State {
	let newState = {
		...state,
	};
	return newState;
}

export type State = {
	// todo
}

const initialState: State = {
	// todo
};

export type Action = {
	type: ActionType;
	item: any;
}

export enum ActionType {
	// todo
}
