import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
    // TODO: Load data of params.id
	return {
		section: {
			title: `${params.id}`,
		},
	};
};
