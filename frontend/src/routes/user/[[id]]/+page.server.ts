import { user } from '$lib/userStore';
import { redirect } from '@sveltejs/kit';

export function load({ params}) {
    let userValue: any;

    user.subscribe((value) => {
      userValue = value;
    });

    console.log(userValue, params.id);

    if (userValue?.id.toString() === params.id) {
      return redirect(302, "/me");
    }
    
}