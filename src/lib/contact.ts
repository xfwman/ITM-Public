export interface ContactFormValues {
  name: string;
  email: string;
  company: string;
  message: string;
  website: string;
}

export interface ContactFormErrors {
  [key: string]: string | undefined;
}

export interface ContactSubmissionResult {
  ok: boolean;
  reference?: string;
  error?: "validation" | "submission";
}

const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export function validateContactValues(
  values: ContactFormValues,
  errors: {
    nameRequired: string;
    emailRequired: string;
    emailInvalid: string;
    messageRequired: string;
  },
): ContactFormErrors {
  const nextErrors: ContactFormErrors = {};

  if (!values.name.trim()) {
    nextErrors.name = errors.nameRequired;
  }

  if (!values.email.trim()) {
    nextErrors.email = errors.emailRequired;
  } else if (!emailPattern.test(values.email.trim())) {
    nextErrors.email = errors.emailInvalid;
  }

  if (!values.message.trim()) {
    nextErrors.message = errors.messageRequired;
  }

  return nextErrors;
}

export async function submitContactRequest(
  values: ContactFormValues,
): Promise<ContactSubmissionResult> {
  if (values.website.trim()) {
    return {
      ok: true,
      reference: "SPAM-BLOCKED",
    };
  }

  await new Promise((resolve) => {
    setTimeout(resolve, 500);
  });

  return {
    ok: true,
    reference: `ITM-${Date.now().toString(36).toUpperCase()}`,
  };
}
