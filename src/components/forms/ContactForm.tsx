"use client";

import { useState } from "react";
import {
  Alert,
  Button,
  Paper,
  Stack,
  Text,
  TextInput,
  Textarea,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconCheck, IconMail, IconX } from "@tabler/icons-react";

import {
  validateContactValues,
  type ContactFormValues,
} from "@/lib/contact";

interface ContactFormProps {
  content: {
    title: string;
    description: string;
    fields: {
      name: string;
      email: string;
      company: string;
      message: string;
      honeypot: string;
    };
    placeholders: {
      name: string;
      email: string;
      company: string;
      message: string;
    };
    validation: {
      nameRequired: string;
      emailRequired: string;
      emailInvalid: string;
      messageRequired: string;
    };
    submit: string;
    submitting: string;
    privacyNote: string;
    successTitle: string;
    successMessage: string;
    failureTitle: string;
    failureMessage: string;
  };
}

type SubmissionState =
  | {
      status: "idle";
    }
  | {
      status: "success";
      reference: string;
    }
  | {
      status: "error";
    };

const initialValues: ContactFormValues = {
  name: "",
  email: "",
  company: "",
  message: "",
  website: "",
};

export function ContactForm({ content }: ContactFormProps) {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submissionState, setSubmissionState] = useState<SubmissionState>({
    status: "idle",
  });

  const form = useForm<ContactFormValues>({
    initialValues,
    validate: (values) => validateContactValues(values, content.validation),
  });

  async function handleSubmit(values: typeof initialValues) {
    setSubmissionState({ status: "idle" });
    setIsSubmitting(true);

    try {
      const response = await fetch("/api/contact", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(values),
      });

      if (!response.ok) {
        throw new Error("Submission failed");
      }

      const result = (await response.json()) as {
        ok: boolean;
        reference?: string;
      };

      if (!result.ok || !result.reference) {
        throw new Error("Unexpected response");
      }

      setSubmissionState({
        status: "success",
        reference: result.reference,
      });
      form.reset();
    } catch (error) {
      console.error(error);
      setSubmissionState({
        status: "error",
      });
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <Paper className="surface-outer-shadow" withBorder p="xl">
      <Stack gap="lg">
        <Stack gap={6}>
          <Text fw={600} size="xl">
            {content.title}
          </Text>
          <Text c="dimmed">{content.description}</Text>
        </Stack>

        {submissionState.status === "success" ? (
          <Alert
            color="teal"
            icon={<IconCheck size={18} />}
            title={content.successTitle}
            variant="light"
          >
            {content.successMessage.replace(
              "{reference}",
              submissionState.reference,
            )}
          </Alert>
        ) : null}

        {submissionState.status === "error" ? (
          <Alert
            color="red"
            icon={<IconX size={18} />}
            title={content.failureTitle}
            variant="light"
          >
            {content.failureMessage}
          </Alert>
        ) : null}

        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack gap="md">
            <TextInput
              label={content.fields.name}
              placeholder={content.placeholders.name}
              withAsterisk
              {...form.getInputProps("name")}
            />
            <TextInput
              label={content.fields.email}
              placeholder={content.placeholders.email}
              type="email"
              withAsterisk
              {...form.getInputProps("email")}
            />
            <TextInput
              label={content.fields.company}
              placeholder={content.placeholders.company}
              {...form.getInputProps("company")}
            />
            <Textarea
              autosize
              label={content.fields.message}
              minRows={5}
              placeholder={content.placeholders.message}
              withAsterisk
              {...form.getInputProps("message")}
            />

            <TextInput
              aria-hidden="true"
              label={content.fields.honeypot}
              tabIndex={-1}
              style={{
                position: "absolute",
                inlineSize: 1,
                blockSize: 1,
                overflow: "hidden",
                clipPath: "inset(50%)",
              }}
              {...form.getInputProps("website")}
            />

            <Text c="dimmed" size="sm">
              {content.privacyNote}
            </Text>

            <Button
              leftSection={<IconMail size={18} />}
              loading={isSubmitting}
              type="submit"
            >
              {isSubmitting ? content.submitting : content.submit}
            </Button>
          </Stack>
        </form>
      </Stack>
    </Paper>
  );
}
