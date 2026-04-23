import { NextResponse } from "next/server";

import {
  submitContactRequest,
  validateContactValues,
  type ContactFormValues,
} from "@/lib/contact";

function toContactValues(value: unknown): ContactFormValues {
  const body = (value ?? {}) as Partial<ContactFormValues>;

  return {
    name: body.name?.toString() ?? "",
    email: body.email?.toString() ?? "",
    company: body.company?.toString() ?? "",
    message: body.message?.toString() ?? "",
    website: body.website?.toString() ?? "",
  };
}

export async function POST(request: Request) {
  const body = toContactValues(await request.json());
  const errors = validateContactValues(body, {
    nameRequired: "name",
    emailRequired: "email",
    emailInvalid: "email",
    messageRequired: "message",
  });

  if (Object.keys(errors).length > 0) {
    return NextResponse.json(
      {
        ok: false,
        error: "validation",
      },
      {
        status: 400,
      },
    );
  }

  const result = await submitContactRequest(body);

  if (!result.ok) {
    return NextResponse.json(result, {
      status: 500,
    });
  }

  return NextResponse.json(result);
}
